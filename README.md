<!DOCTYPE html>
<html>

<head>
    <title>PR-Bot</title>
    <style>
        /* Add your custom CSS styles here */
    </style>
</head>

<body>
    <h1>PR-Bot</h1>
    <p>Lambda function to handle Bitbucket webhook payloads, extract relevant information, and send notifications to Microsoft Teams, saving you time on opening PR review requests manually.</p>

    <h2>Prerequisites</h2>
    <p>Before using this PR-Bot, ensure you have the following prerequisites installed and configured:</p>
    <ul>
        <li>Rust: The programming language used to build the Lambda function. <a href="https://www.rust-lang.org/tools/install">Installation Guide</a></li>
        <li>AWS CLI (optional but recommended): This will save time during deployment. <a href="https://docs.aws.amazon.com/cli/latest/userguide/getting-started-install.html">Installation Guide</a></li>
        <li>Cargo Lambda: A tool for building Rust AWS Lambda functions. For more documentation, please refer to <a href="https://www.cargo-lambda.info/">Cargo Lambda</a></li>
        <li>Microsoft Teams: Access to a Microsoft Teams channel where you want to receive notifications.</li>
        <li>BitBucket Account / Repo: The scope from where you will be sending notifications to Microsoft Teams.</li>
    </ul>

    <h2>Configuration</h2>

    <h3>Microsoft Teams Webhook</h3>
    <p>Configure a webhook in your Microsoft Teams channel where you want to receive notifications. Take note of the generated webhook URL.</p>

    <h3>Lambda Function Configuration</h3>
    <ol>
        <li>Clone this repository to your local environment.</li>
        <li>Change TEAM_URL with your webhook from Microsoft Teams or set it as an env variable in Lambda.</li>
        <li>Open the template.json file and update the CHANNEL_ID with the correct channel ID in the correct format.</li>
        <p>To find the correct ID, go to Microsoft Teams channel, extract URL from the channel. The ID you need will be in the format of <code>&lt;some numbers&gt;:&lt;some more numbers&gt;@thread.tacv2</code>. Other channels may have a similar format, but the ending could be <code>@thread.skype</code>. Update the ID in the code.</p>
        <li>Modify the deploy.sh script to ensure that it has the appropriate role permissions for AWS Lambda. It is also possible to manually upload the zip file to Lambda from AWS Console.</li>
    </ol>

    <h2>Usage</h2>
    <p>To generate a zip file:</p>
    <code>
        cargo lambda build --release --output-format zip
    </code>
    <p>And to upload from CLI:</p>
    <code>
        ./deploy.sh
    </code>

</body>

</html>
