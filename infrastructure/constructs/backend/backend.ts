import { IgnoreMode } from "aws-cdk-lib";
import * as ec2 from "aws-cdk-lib/aws-ec2";
import * as ecs from "aws-cdk-lib/aws-ecs";
import * as iam from "aws-cdk-lib/aws-iam";
import * as logs from "aws-cdk-lib/aws-logs";
import * as secretsmanager from "aws-cdk-lib/aws-secretsmanager";
import * as elbv2 from "aws-cdk-lib/aws-elasticloadbalancingv2";
import * as servicediscovery from "aws-cdk-lib/aws-servicediscovery";
import { Construct } from "constructs";
import * as ecr from "aws-cdk-lib/aws-ecr";
import * as cdk from "aws-cdk-lib";

export type BackendProps = {
	cluster: ecs.ICluster;
	service: string;
	healthCheckPath: string;
	containerEnvironment?: { [key: string]: string };
	containerSecrets?: { [key: string]: ecs.Secret };
	inlineRolePolicies?: iam.RoleProps["inlinePolicies"];
	vpc: ec2.IVpc;
};

export class Backend extends Construct {
	public readonly service: ecs.FargateService;
	public readonly role: iam.Role;
	public readonly loadBalancer: elbv2.ApplicationLoadBalancer;

	constructor(scope: Construct, id: string, props: BackendProps) {
		super(scope, id);

		const { cluster, inlineRolePolicies = {}, vpc } = props;

		const fillerSecret = secretsmanager.Secret.fromSecretNameV2(
			this,
			"uniswapx-filler-secret",
			"uniswapx-filler/config",
		);

		// Create a security group for the services
		const serviceSecurityGroup = new ec2.SecurityGroup(
			this,
			"ServiceSecurityGroup",
			{
				vpc,
				description: "Security group for uniswapx filler services",
				allowAllOutbound: true,
			},
		);

		// Create namespace for service discovery
		const namespace = new servicediscovery.PrivateDnsNamespace(
			this,
			"MonitoringNamespace",
			{
				name: "monitoring.local",
				vpc,
			},
		);

		// Create roles for the services
		this.role = new iam.Role(this, `${id}-role`, {
			assumedBy: new iam.ServicePrincipal("ecs-tasks.amazonaws.com"),
			managedPolicies: [
				iam.ManagedPolicy.fromAwsManagedPolicyName(
					"service-role/AmazonECSTaskExecutionRolePolicy",
				),
			],
			inlinePolicies: {
				...inlineRolePolicies,
			},
		});

		const taskDefinition = new ecs.TaskDefinition(this, "api", {
			family: props.service,
			compatibility: ecs.Compatibility.EC2_AND_FARGATE,
			cpu: "1024",
			memoryMiB: "2048",
			networkMode: ecs.NetworkMode.AWS_VPC,
			taskRole: this.role,
		});

		taskDefinition.addToTaskRolePolicy(
			new iam.PolicyStatement({
				actions: ["secretsmanager:GetSecretValue"],
				resources: [fillerSecret.secretArn],
			}),
		);

		const image = ecs.ContainerImage.fromAsset("./", {
			file: "Dockerfile",
			buildArgs: {
				SERVICE: props.service,
			},
			ignoreMode: IgnoreMode.DOCKER,
		});

		// Add main application container with metrics endpoint
		const mainContainer = taskDefinition.addContainer(`${id}-backend`, {
			image,
			memoryLimitMiB: 2048,
			environment: {
				...props.containerEnvironment,
				CHAIN_ID: "999",
				RUST_LOG: "INFO",
				ORDER_TYPE: "Dutch_V2",
				EXECUTOR_ADDRESS: "0x86Fba278335de371032Aa3Cc9052f11774EBB060",
				BID_PERCENTAGE: 10,
				WSS: "wss://rpc.purroofgroup.com",
				UNISWAPX_API: "https://2s04u0xev3.execute-api.us-west-2.amazonaws.com/prod",
				ROUTING_API: "https://2s04u0xev3.execute-api.us-west-2.amazonaws.com/prod",
				REACTOR_ADDRESS: "0xB274d5F4b833b61B340b654d600A864fB604a87c",
			},
			secrets: {
				PRIVATE_KEY: ecs.Secret.fromSecretsManager(fillerSecret, "PRIVATE_KEY"),
			},
			command: [
				"/bin/sh",
				"-c",
				"/app/uniswapx-artemis --wss $WS_RPC_URL --private-key $PRIVATE_KEY --bid-percentage $BID_PERCENTAGE --executor-address $EXECUTOR_ADDRESS --chain-id $CHAIN_ID --order-type $ORDER_TYPE --uniswapx-api-url $UNISWAPX_API_URL --routing-api $ROUTING_API --reactor-address $REACTOR_ADDRESS",
			],
			portMappings: [{ containerPort: 3000 }, { containerPort: 3001 }],
			logging: ecs.LogDriver.awsLogs({
				streamPrefix: "hypurr-pawswapx-filler-backend",
				logRetention: logs.RetentionDays.THREE_DAYS,
			}),
			essential: true,
		});

		// Create the main service with service discovery
		this.service = new ecs.FargateService(this, `${id}-service`, {
			cluster: cluster,
			serviceName: "pawswapx-filler",
			taskDefinition: taskDefinition,
			securityGroups: [serviceSecurityGroup],
			assignPublicIp: false,
			minHealthyPercent: 0,
			maxHealthyPercent: 100,
			desiredCount: 1,
			circuitBreaker: {
				rollback: false,
			},
			deploymentController: {
				type: ecs.DeploymentControllerType.ECS,
			},
			cloudMapOptions: {
				name: "uniswapx-filler",
				cloudMapNamespace: namespace,
				dnsRecordType: servicediscovery.DnsRecordType.A,
			},
		});
	}
}
