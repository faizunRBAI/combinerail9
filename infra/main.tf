# ── Data sources ──────────────────────────────────────────────────────────────

data "aws_vpc" "default" {
  default = true
}

data "aws_subnets" "default" {
  filter {
    name   = "vpc-id"
    values = [data.aws_vpc.default.id]
  }
}

data "aws_ami" "ubuntu" {
  most_recent = true
  owners      = ["099720109477"] # Canonical

  filter {
    name   = "name"
    values = ["ubuntu/images/hvm-ssd/ubuntu-jammy-22.04-amd64-server-*"]
  }

  filter {
    name   = "virtualization-type"
    values = ["hvm"]
  }
}

# ── SSH Key Pair ───────────────────────────────────────────────────────────────

resource "aws_key_pair" "main" {
  key_name   = "${var.project_name}-key"
  public_key = var.ssh_public_key

  tags = {
    Project   = var.project_name
    ManagedBy = "udap"
  }
}

# ── Security Group ─────────────────────────────────────────────────────────────

resource "aws_security_group" "app" {
  name        = "${var.project_name}-sg"
  description = "Allow HTTP (80) and SSH (22) inbound traffic"
  vpc_id      = data.aws_vpc.default.id

  ingress {
    description = "HTTP"
    from_port   = 80
    to_port     = 80
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }

  ingress {
    description = "SSH"
    from_port   = 22
    to_port     = 22
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }

  egress {
    from_port   = 0
    to_port     = 0
    protocol    = "-1"
    cidr_blocks = ["0.0.0.0/0"]
  }

  tags = {
    Name      = "${var.project_name}-sg"
    Project   = var.project_name
    ManagedBy = "udap"
  }
}

# ── EC2 Instance ───────────────────────────────────────────────────────────────

resource "aws_instance" "app" {
  ami                    = data.aws_ami.ubuntu.id
  instance_type          = var.instance_type
  key_name               = aws_key_pair.main.key_name
  subnet_id              = data.aws_subnets.default.ids[0]
  vpc_security_group_ids = [aws_security_group.app.id]

  root_block_device {
    volume_size = 20
    volume_type = "gp3"
  }

  tags = {
    Name      = var.project_name
    Project   = var.project_name
    ManagedBy = "udap"
  }
}

# ── Elastic IP ─────────────────────────────────────────────────────────────────

resource "aws_eip" "app" {
  instance = aws_instance.app.id
  domain   = "vpc"

  tags = {
    Name      = "${var.project_name}-eip"
    Project   = var.project_name
    ManagedBy = "udap"
  }
}
