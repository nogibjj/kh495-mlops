# AWS Lambda + EFS + Onnx Serverless Inference

## Useage

**Configure AWS IAM Permissions**

1. Create an IAM User policy for "LambdaDev" with `AWSLambda_FullAccess` permissions and added custom inline permission config
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
2. Add "LambdaDev" access keys to your local ~/.aws/credentials file along with environment variables: 
* aws_access_key_id
* aws_secret_access_key
* aws_role_arn (NB: this is copied from the IAM user summary and is formatted as arn:aws:iam::<aws_acct>:user/<iam_user>)
* region 

3. Create an IAM Role policy named "EFSxLambda" with `AWSLambdaBasicExecutionRole` + `AWSLambdaVPCAccessExecutionRole` + `AmazonElasticFileSystemClientFullAccess` permissions

**Deploy Lambda Fxn from Developer Environment (i.e. this repo)**

```
# Install cargo-lambda
$ cd onnx-aws
$ python3 -m venv ~/.venv
$ source ~/.venv/bin/activate

# Build Binary 
$ make release

# Deploy to AWS
$ make deploy
```

**Setup Cloud9**

1. Launch Cloud9 console and create an environment with "New EC2 Instance" + "SSM Connection" + "default VPC"
2. Once provisioned, click on env details > EC2 instance > manage EC2 instance. Make note of the associated security group listed.

**Setup EFS**

1. Launch AWS EFS console and provision EFS instance
2. Once provisioned, click on file system name > network. Make note of the security group id listed. 
3. Under EFS access points tab > add access point > create with following settings:
  * Name: LambdaEFS
  * Root directory path: `/` (by default root is /mnt/efs)
  * Posix User ID: 1000
  * Posix Group ID: 1000
  * Owner User ID: 1000
  * Owner Group ID: 1000
  * Access point permissions: 0777

**Configure Security Groups**

1. From EC2 console > security groups > new security group: 
  * Name = onnxLambda
  * Description = Mangage onnx-aws lambda access
  * VPC = same as EFS (default)
2. To enable EFS --> Lambda: click onnxLambda security group ID > edit inbound rules > Set: type = NFS, protocol = TCP, port range = 2049, source = custom > add the EFS security group ID (from the drop down box).
2. To enable EFS --> Cloud9: click Cloud9 security group ID > edit inbound rules > Set: type = NFS, protocol = TCP, port range = 2049, source = custom > add the EFS security group ID (from the drop down box).
3. To enable Lambda --> EFS and Cloud9 --> EFS: click EFS security group ID > edit inbound rules > add inbound rule for each:
  * Set: type = NFS, protocol = TCP, port range = 2049, source = custom > add the EFS security group ID (from the drop down box)
  * Set: type = NFS, protocol = TCP, port range = 2049, source = custom > add the Cloud9 security group ID (from the drop down box)

**Configure Lambda for EFS**

1. Configuration > Permissions > edit execution role > select "EFSxLambda" from above
2. Configuration > VPC > select default VPC (to match EFS)
3. Configuration > VPC > edit security group to EFSxLambda
4. Configuration > File Systems > add EFS + access point + local mount path = /mnt/efs
5. Configuration > Env Variables > LD_LIBRARY_PATH = /mnt/efs

**Mount EFS to Cloud9**

1. Launch Cloud9 environment
2. Run the following commands to mount to /mnt/efs

```
# Docs: https://repost.aws/knowledge-center/efs-mount-automount-unmount-steps

# Install efs-ultils (https://docs.aws.amazon.com/efs/latest/ug/installing-amazon-efs-utils.html)
sudo yum install -y amazon-efs-utils

sudo mkdir -p /mnt/efs

# From EFS > Attach > copy EFS helper command
sudo mount -t efs -o tls <AWS_FS_ID>:/ /mnt/efs

# Switch ownership permissions
sudo chown ec2-user:ec2-user /mnt/efs
sudo chmod 755 /mnt/efs

# Download files locally
# squeezenet model
wget https://github.com/onnx/models/blob/main/vision/classification/squeezenet/model/squeezenet1.0-12.onnx?raw=true -O squeezenet1.0-12.onnx

# onnx runtime v1.8.1
wget https://github.com/microsoft/onnxruntime/releases/download/v1.8.1/onnxruntime-linux-x64-1.8.1.tgz
tar -xvzf onnxruntime-linux-x64-1.8.1.tgz
rm onnxruntime-linux-x64-1.8.1.tgz

# copy to /mnt/efs
cp onnxruntime-linux-x64-1.8.1/lib/libonnxruntime.so.1.8.1 /mnt/efs 
cp squeezenet1.0-12.onnx /mnt/efs 

# check files have been moved over
cd /mnt/efs
ls
```

**Invoke from local dev environment**
```
make invoke

# Response
cargo lambda invoke --remote \
                --data-ascii '{"name": "onnx"}' \
                --output-format json \
                onnx-aws
{
  "files": "\n/mnt/efs/libonnxruntime.so.1.8.1\n/mnt/efs/squeezenet1.0-12.onnx",
  "msg": "Hello, onnx!",
  "req_id": "01a14795-6961-410a-8496-1406d9ab4c53",
  "scores": [
    0.000045440578,
    0.0038458686,
    0.0001249467,
    0.0011804511,
    0.00131694
  ]
}
```

## References
* [AWS EFS + Lambda Guide](https://aws.amazon.com/blogs/compute/using-amazon-efs-for-aws-lambda-in-your-serverless-applications/)
* [Mount EC2 to EFS](https://docs.aws.amazon.com/efs/latest/ug/mounting-fs-mount-helper-ec2-linux.html)
* [Noah's Repo](https://github.com/noahgift/rust-mlops-template/tree/main/onnx-efs-lambda)