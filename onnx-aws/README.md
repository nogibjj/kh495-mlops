# AWS Lambda + EFS + Onnx Serverless Inference

## Useage

**Configure AWS IAM Role**

1. Create an AWS IAM User policy with `AWSLambda_FullAccess` permissions and custom permission config
```
{
    "Version": "2012-10-17",
    "Statement": [
        {
            "Sid": "PermissionsToDeploy",
            "Effect": "Allow",
            "Action": [
                "iam:CreateRole",
                "iam:CreatePolicy",
                "iam:PutRolePolicy",
                "iam:AttachRolePolicy",
                "iam:UpdateAssumeRolePolicy"
            ],
            "Resource": "*"
        }
    ]
}
```
2. With above User policy, set your ~/.aws/credentials file with environment variables: `aws_access_key_id`, `aws_secret_access_key`, `aws_role_arn`. 

*Note: `aws_role_arn` is copied from the IAM user summary and is formatted as arn:aws:iam::<aws_acct>:user/<iam_user>*

**Lambda role with basic execution +**
```
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Effect": "Allow",
      "Action": [
        "ec2:DescribeNetworkInterfaces",
        "ec2:CreateNetworkInterface",
        "ec2:DeleteNetworkInterface",
        "ec2:DescribeInstances",
        "ec2:AttachNetworkInterface"
      ],
      "Resource": "*"
    }
  ]
}
```

**Configure EFS**
1. From AWS console, provision EFS instance
2. Add access point `/mnt/efs` to above EFS

**Install Cargo Lambda into Virtual Env**

```
$ cd onnx-aws
$ python3 -m venv ~/.venv
$ source ~/.venv/bin/activate
```

**Build Binary**

```
$ make release
```

**Deploy Lambda Fxn to AWS**
```
$ make deploy
```

**Deploy fxn + add VPC + filesystem**

**Ec2 modify security group rules**

**Create Cloud9 Instance and set EC2 security group to one from above**


```

# Install efs-ultils (https://docs.aws.amazon.com/efs/latest/ug/installing-amazon-efs-utils.html)
sudo yum install -y amazon-efs-utils

# Mount efs to ec2 (https://docs.aws.amazon.com/efs/latest/ug/mounting-fs-mount-helper-ec2-linux.html)
sudo mkdir efs

# From EFS "Attach" copy EFS helper command
sudo mount -t efs -o tls <AWS_FS_ID>:/ efs
```



## References
* [AWS EFS + Lambda Guide](https://aws.amazon.com/blogs/compute/using-amazon-efs-for-aws-lambda-in-your-serverless-applications/)