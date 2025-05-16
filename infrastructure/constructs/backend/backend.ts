import { IgnoreMode } from "aws-cdk-lib";
import * as ec2 from "aws-cdk-lib/aws-ec2";
import * as ecs from "aws-cdk-lib/aws-ecs";
import * as iam from "aws-cdk-lib/aws-iam";
import * as logs from "aws-cdk-lib/aws-logs";
import * as secretsmanager from "aws-cdk-lib/aws-secretsmanager";
import { Construct } from "constructs";

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

  constructor(scope: Construct, id: string, props: BackendProps) {
    super(scope, id);

    const { cluster, inlineRolePolicies = {}, vpc } = props;

    const artemisSecret = secretsmanager.Secret.fromSecretNameV2(
      this,
      "uniswapx-artemis-secret",
      "uniswapx-artemis/config"
    );

    // Create a security group for the services
    const serviceSecurityGroup = new ec2.SecurityGroup(
      this,
      "ServiceSecurityGroup",
      {
        vpc,
        description: "Security group for backendservices",
        allowAllOutbound: true,
      }
    );

    // Create roles for the services
    this.role = new iam.Role(this, `${id}-role`, {
      assumedBy: new iam.ServicePrincipal("ecs-tasks.amazonaws.com"),
      managedPolicies: [
        iam.ManagedPolicy.fromAwsManagedPolicyName(
          "service-role/AmazonECSTaskExecutionRolePolicy"
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
        resources: [artemisSecret.secretArn],
      })
    );

    const image = ecs.ContainerImage.fromAsset("./", {
      file: "Dockerfile",
      buildArgs: {
        SERVICE: props.service,
      },
      ignoreMode: IgnoreMode.DOCKER,
    });

    taskDefinition.addContainer(`${id}-backend`, {
      image,
      memoryLimitMiB: 2048,
      environment: {
        ...props.containerEnvironment,
        WS: "wss://rpc.purroofgroup.com",
        BID_PERCENTAGE: "1",
        ORDER_TYPE: "DutchV2",
        CHAIN_ID: "999",
        RUST_LOG: "INFO",
        EXECUTOR_ADDRESS: "0x9304a794665d748cA940EdA8026a2BF243F6e267",
      },
      secrets: {
        PRIVATE_KEY: ecs.Secret.fromSecretsManager(
          artemisSecret,
          "PRIVATE_KEY"
        ),
      },
      command: [
        "/bin/sh",
        "-c",
        "/app/uniswapx-artemis --private-key $PRIVATE_KEY --chain-id $CHAIN_ID --ws $WS --bid-percentage $BID_PERCENTAGE --order-type $ORDER_TYPE --executor-address $EXECUTOR_ADDRESS",
      ],
      portMappings: [{ containerPort: 1559 }],
      logging: ecs.LogDriver.awsLogs({
        streamPrefix: "hypurr-uniswapx-artemis",
        logRetention: logs.RetentionDays.THREE_DAYS,
      }),
      essential: true,
    });

    // Create the main service with service discovery
    this.service = new ecs.FargateService(this, `${id}-service`, {
      cluster: cluster,
      serviceName: "uniswapx-artemis",
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
    });
  }
}
