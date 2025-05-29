import { App } from "aws-cdk-lib";
import { Liquidator } from "./liquidator";
import { Cluster } from "./shared/cluster";

const app = new App();

const cluster = new Cluster(app, "imported-hypurr-liquidator-cluster", {
  env: {
    account: process.env.CDK_DEFAULT_ACCOUNT,
    region: process.env.CDK_DEFAULT_REGION,
  },
});

new Liquidator(app, "hypurr-liquidator-frax", {
  cluster: cluster.cluster,
  vpc: cluster.vpc,
  policies: {},
});
