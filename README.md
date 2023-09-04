# PR-Bot
 Lambda function to handle Bitbucket webhook payloads, extract relevant information and send notifications to Microsoft Teams, saving you time on opening PR review requests manually.

# Prerequisites
  
 Before using this PR-Bot, ensure you have the following prerequisites installed and configured:

   Rust: The programming language used to build the Lambda function. https://www.rust-lang.org/tools/install
   AWS CLI (optional but recommended): This will save time during deployment. https://docs.aws.amazon.com/cli/latest/userguide/getting-started-install.html
   Cargo Lambda: A tool for building Rust AWS Lambda functions. For more documentation please refer to https://www.cargo-lambda.info/
   Microsoft Teams: Access to a Microsoft Teams channel where you want to receive notifications.
   BitBucket Account / Repo: The scope from where you will be sending notifications to Microsoft Teams

# Configuration

## Microsoft Teams Webhook
 Configure a webhook in your Microsoft Teams channel where you want to receive notifications. Take note of the generated webhook URL.

## Lambda Function Configuration
 Clone this repository to your local environment.

 Change TEAM_URL with your webhook from Microsoft Teams or set it as env variable in Lambda.

 Open the template.json file and update the CHANNEL_ID with the correct channel ID in the correct format.
 To find the correct ID, go to Microsoft Teams channel, extract URL from the channel. 
 The ID you need will be in the format of <some numbers>:<some more numbers>@thread.tacv2. 
 Other channels may have a similar format, but the ending could be @thread.skype.
 Update the ID in the code.
 
 Modify the deploy.sh script to ensure that it has the appropriate role permissions for AWS Lambda.
 It is also possible to manually upload the zip file to Lambda from AWS Console.

# Usage

To generate zip file:
```
 cargo lambda build --release --output-format zip
```
And to upload from CLI: 
```
./deploy.sh
```
 
