#!/bin/sh
aws lambda create-function --function-name name-of-your-function \
     --runtime provided.al2 \
     --role arn:aws:iam::YOU-ROLE \
     --handler rust.handler \
     --zip-file fileb://target/lambda/webhookrs/bootstrap.zip
