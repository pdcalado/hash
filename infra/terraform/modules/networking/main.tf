resource "aws_vpc" "main" {
  # IP address range 10.0.0.0 - 10.0.255.255 (131072 addresses)
  # We will have 10.0.0.0 - 10.0.127.0 contain the private subnet
  # and have 10.0.128.0 - 10.0.255.0 addresses contain the public subnet
  cidr_block           = "10.0.0.0/16"
  enable_dns_support   = true
  enable_dns_hostnames = true
  tags                 = { Name = "${var.prefix}-vpc" }
}

# Flow logs in VPC
resource "aws_flow_log" "flow_log" {
  tags = { Name = "${var.prefix}-flowvpc" }

  iam_role_arn    = aws_iam_role.flow_log.arn
  log_destination = aws_cloudwatch_log_group.flow_log.arn
  traffic_type    = "ALL"
  vpc_id          = aws_vpc.main.id
}

resource "aws_cloudwatch_log_group" "flow_log" {
  name = "${var.prefix}-flowlogvpc"
}

resource "aws_iam_role" "flow_log" {
  name = "${var.prefix}-flowlogrole"

  assume_role_policy = <<EOF
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Sid": "",
      "Effect": "Allow",
      "Principal": {
        "Service": "vpc-flow-logs.amazonaws.com"
      },
      "Action": "sts:AssumeRole"
    }
  ]
}
EOF
}

resource "aws_iam_role_policy" "flow_log" {
  name = "${var.prefix}-iamflowlog"
  role = aws_iam_role.flow_log.id

  policy = <<EOF
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Action": [
        "logs:CreateLogGroup",
        "logs:CreateLogStream",
        "logs:PutLogEvents",
        "logs:DescribeLogGroups",
        "logs:DescribeLogStreams"
      ],
      "Effect": "Allow",
      "Resource": "*"
    }
  ]
}
EOF
}

# Gateway to allow communication between VPC and the internet
resource "aws_internet_gateway" "igw" {
  vpc_id = aws_vpc.main.id
  tags = {
    Name = "${var.prefix}-igw"
  }
}

####################################
# Private subnet
####################################

# Private routing table used for the private subnet only
resource "aws_route_table" "rtpriv" {
  vpc_id = aws_vpc.main.id
  # Note: terraform automatically creates the "local" route on the VPC's CIDR block.
  # See: https://registry.terraform.io/providers/hashicorp/aws/latest/docs/resources/route_table
  tags = {
    Name = "${var.prefix}-rtpriv"
    Tier = "private"
  }
}

# Create as many private subnets as availability zones specified
resource "aws_subnet" "snpriv" {
  count  = length(var.region_az_names)
  vpc_id = aws_vpc.main.id

  # Turn into a 10.0.0.0/24
  # 10.0.x.0 - 10.0.x.255 (256 addresses)
  cidr_block        = cidrsubnet(aws_vpc.main.cidr_block, 8, count.index)
  availability_zone = var.region_az_names[count.index]
  tags = {
    Name    = "${var.prefix}-snpriv"
    NameIdx = "${var.prefix}-snpriv${count.index + 1}"
    Tier    = "private"
  }
}

# Routing association that allows traffic within the private subnet
resource "aws_route_table_association" "snpriv" {
  count = length(var.region_az_names)

  subnet_id      = aws_subnet.snpriv[count.index].id
  route_table_id = aws_route_table.rtpriv.id
}


####################################
# Public subnet
####################################

# Public routing table used for the public subnet
resource "aws_route_table" "rtpub" {
  depends_on = [aws_internet_gateway.igw]
  vpc_id     = aws_vpc.main.id
  # Note: terraform automatically creates the "local" route on the VPC's CIDR block
  # See: https://registry.terraform.io/providers/hashicorp/aws/latest/docs/resources/route_table
  route {
    cidr_block = "0.0.0.0/0"
    gateway_id = aws_internet_gateway.igw.id
  }
  tags = {
    Name = "${var.prefix}-rtpub"
    Tier = "public"
  }
}

# Create as many public subnets as availability zones specified
resource "aws_subnet" "snpub" {
  count  = length(var.region_az_names)
  vpc_id = aws_vpc.main.id

  # Turn into a 10.0.0.0/24
  # 10.1.x.0 - 10.1.x.255 (256 addresses)
  cidr_block        = cidrsubnet(aws_vpc.main.cidr_block, 8, 128 + count.index)
  availability_zone = var.region_az_names[count.index]
  tags = {
    Name    = "${var.prefix}-snpub"
    NameIdx = "${var.prefix}-snpub${count.index + 1}"
    Tier    = "public"
  }
}

# Routing association that allows traffic to go through the internet gateway
resource "aws_route_table_association" "snpub" {
  count = length(var.region_az_names)

  subnet_id      = aws_subnet.snpub[count.index].id
  route_table_id = aws_route_table.rtpub.id
}

####################################
# PrivateLink interface endpoints
# Allows for services in private subnets to connect to particular AWS services
# For full list see: https://docs.aws.amazon.com/vpc/latest/privatelink/integrated-services-vpce-list.html
####################################

# Security group that allows for private link interface endpoint connections
resource "aws_security_group" "vpce" {
  name        = "${var.prefix}-sgvpce"
  description = "VPC Endpoint"
  vpc_id      = aws_vpc.main.id

  ingress {
    from_port   = 443
    to_port     = 443
    protocol    = "tcp"
    description = "Allow endpoint connections"
    cidr_blocks = [aws_vpc.main.cidr_block]
  }

  ingress {
    from_port   = 587
    to_port     = 587
    protocol    = "tcp"
    description = "Allow smtp endpoint connections"
    cidr_blocks = [aws_vpc.main.cidr_block]
  }

  ingress {
    from_port   = 6379
    to_port     = 6379
    protocol    = "tcp"
    description = "Allow Redis connections"
    cidr_blocks = [aws_vpc.main.cidr_block]
  }

  tags = { Name = "${var.prefix}-sgvpce" }
}

# Allow private endpoint to fetch S3 content
# This is very important, as ECR layers are stored in S3
# Beware that this endpoint must be a Gateway endpoint and not an interface!
# https://docs.aws.amazon.com/AmazonS3/latest/userguide/privatelink-interface-endpoints.html
resource "aws_vpc_endpoint" "s3" {
  vpc_id            = aws_vpc.main.id
  service_name      = "com.amazonaws.${var.region}.s3"
  vpc_endpoint_type = "Gateway"

  route_table_ids = [
    aws_route_table.rtpriv.id
  ]

  tags = { Name = "${var.prefix}-vpces3" }
}

# Allow fetching EC2 API access within the private subnet
# Used for various Fargate related operations (required)
resource "aws_vpc_endpoint" "ec2" {
  vpc_id              = aws_vpc.main.id
  service_name        = "com.amazonaws.${var.region}.ec2"
  vpc_endpoint_type   = "Interface"
  subnet_ids          = aws_subnet.snpriv[*].id
  private_dns_enabled = true
  security_group_ids = [
    aws_vpc.main.default_security_group_id,
    aws_security_group.vpce.id
  ]
}

# Allow fetching ECR data within the private subnet
# Used to pull OCI containers
resource "aws_vpc_endpoint" "ecrdkr" {
  vpc_id              = aws_vpc.main.id
  service_name        = "com.amazonaws.${var.region}.ecr.dkr"
  vpc_endpoint_type   = "Interface"
  subnet_ids          = aws_subnet.snpriv[*].id
  private_dns_enabled = true
  security_group_ids = [
    aws_vpc.main.default_security_group_id,
    aws_security_group.vpce.id
  ]
}

# Allow fetching ECR data within the private subnet
# Used to pull OCI containers
resource "aws_vpc_endpoint" "ecrapi" {
  vpc_id              = aws_vpc.main.id
  service_name        = "com.amazonaws.${var.region}.ecr.api"
  vpc_endpoint_type   = "Interface"
  subnet_ids          = aws_subnet.snpriv[*].id
  private_dns_enabled = true
  security_group_ids = [
    aws_vpc.main.default_security_group_id,
    aws_security_group.vpce.id
  ]
}

# Allow CloudWatch as a logging endpoint.
resource "aws_vpc_endpoint" "logs" {
  vpc_id              = aws_vpc.main.id
  service_name        = "com.amazonaws.${var.region}.logs"
  vpc_endpoint_type   = "Interface"
  subnet_ids          = aws_subnet.snpriv[*].id
  private_dns_enabled = true
  security_group_ids = [
    aws_vpc.main.default_security_group_id,
    aws_security_group.vpce.id
  ]
}

# Allow secrets to be fetched from the private subnet
# SSM contain the Parameter Store, which stores secrets.
resource "aws_vpc_endpoint" "ssm" {
  vpc_id              = aws_vpc.main.id
  service_name        = "com.amazonaws.${var.region}.ssm"
  vpc_endpoint_type   = "Interface"
  subnet_ids          = aws_subnet.snpriv[*].id
  private_dns_enabled = true
  security_group_ids = [
    aws_vpc.main.default_security_group_id,
    aws_security_group.vpce.id
  ]
}

# Allow RDS connections
resource "aws_vpc_endpoint" "rds" {
  vpc_id              = aws_vpc.main.id
  service_name        = "com.amazonaws.${var.region}.rds"
  vpc_endpoint_type   = "Interface"
  subnet_ids          = aws_subnet.snpriv[*].id
  private_dns_enabled = true
  security_group_ids = [
    aws_vpc.main.default_security_group_id,
    aws_security_group.vpce.id
  ]
}

# Allow Redis connections from the private subnet
# Used by both private subnet containers (realtime and search-loader)
resource "aws_vpc_endpoint" "elasticache" {
  vpc_id              = aws_vpc.main.id
  service_name        = "com.amazonaws.${var.region}.elasticache"
  vpc_endpoint_type   = "Interface"
  subnet_ids          = [aws_subnet.snpriv[0].id, aws_subnet.snpub[1].id]
  private_dns_enabled = true
  security_group_ids = [
    aws_vpc.main.default_security_group_id,
    aws_security_group.vpce.id
  ]
}

# Allow SES from containers in the public subnet
resource "aws_vpc_endpoint" "email" {
  vpc_id              = aws_vpc.main.id
  service_name        = "com.amazonaws.${var.region}.email-smtp"
  vpc_endpoint_type   = "Interface"
  subnet_ids          = aws_subnet.snpriv[*].id
  private_dns_enabled = true
  security_group_ids = [
    aws_vpc.main.default_security_group_id,
    aws_security_group.vpce.id
  ]
}
