resource "aws_ecrpublic_repository" "ecr_search" {
  provider = aws.us_east_1

  repository_name = "9c3de1338c9f1c8647ddd2fb7dd32856_search"
}