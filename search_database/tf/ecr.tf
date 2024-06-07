resource "aws_ecrpublic_repository" "ecr_search_migrations" {
  provider = aws.us_east_1

  repository_name = "548aa238cf557a6669a82a3579832051_search_migrations"
}