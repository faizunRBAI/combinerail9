variable "project_name" {
  description = "Project name — used as resource name prefix and tag value."
  type        = string
}

variable "ssh_public_key" {
  description = "RSA public key injected by the platform (SSH_PUBLIC_KEY secret)."
  type        = string
}

variable "instance_type" {
  description = "EC2 instance type."
  type        = string
  default     = "t3.micro"
}

variable "region" {
  description = "AWS region."
  type        = string
  default     = "us-east-1"
}
