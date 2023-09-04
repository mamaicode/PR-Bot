# PR-Bot

Lambda function to handle Bitbucket webhook payloads, extract relevant information, and send notifications to Microsoft Teams, saving you time on opening PR review requests manually.

![Example of the notification](https://github.com/scirin/PR-Bot/blob/main/home/wwadmin/Desktop/PR-Bot/example.png)

## Prerequisites

Before using this PR-Bot, ensure you have the following prerequisites installed and configured:

- Rust: The programming language used to build the Lambda function. [Installation Guide](https://www.rust-lang.org/tools/install)
- AWS CLI (optional but recommended): This will save time during deployment. [Installation Guide](https://docs.aws.amazon.com/cli/latest/userguide/getting-started-install.html)
- Cargo Lambda: A tool for building Rust AWS Lambda functions. For more documentation, please refer to [Cargo Lambda](https://www.cargo-lambda.info/)
- Microsoft Teams: Access to a Microsoft Teams channel where you want to receive notifications.
- BitBucket Account / Repo: The scope from where you will be sending notifications to Microsoft Teams.

## Configuration

### Microsoft Teams Webhook

1. Configure a webhook in your Microsoft Teams channel where you want to receive notifications. Take note of the generated webhook URL.

### Lambda Function Configuration

1. Clone this repository to your local environment.

2. Change TEAM_URL with your webhook from Microsoft Teams or set it as an env variable in Lambda.

3. Open the template.json file and update the CHANNEL_ID with the correct channel ID in the correct format.

   To find the correct ID, go to Microsoft Teams channel, extract URL from the channel. The ID you need will be in the format of `<some numbers>:<some more numbers>@thread.tacv2`. Other channels may have a similar format, but the ending could be `@thread.skype`. Update the ID in the code.

4. Modify the deploy.sh script to ensure that it has the appropriate role permissions for AWS Lambda. It is also possible to manually upload the zip file to Lambda from AWS Console.

## Usage

To generate a zip file:

```
cargo lambda build --release --output-format zip
```

And to upload from CLI:

```
./deploy.sh
```

After the Lambda function is uploaded to AWS, create an API gateway.
Copy the link and paste it in your webhoooks on BitBucket, when triggered you will receive a card in Microsoft Teams.

I highly advise you to increase the function timeout ( I have set it to 8 seconds )

Thank you to leon3s [https://github.com/leon3s] for his assistance and creativity
