FROM ubuntu
WORKDIR /app

RUN apt-get update -y \
  && apt update -y \
  && apt-get upgrade -y \
  && apt-get install -y mysql-client \
  && apt upgrade openssl -y \
  && apt-get install -y sqlite3