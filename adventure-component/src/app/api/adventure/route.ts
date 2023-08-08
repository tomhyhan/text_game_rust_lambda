import { NextResponse } from 'next/server'
import { LambdaClient, InvokeCommand } from '@aws-sdk/client-lambda';

export async function GET() {
    try {
        const region = process.env.AWS_LAMBDA_REGION ?? 'us-east-1';
        const accessKeyId = process.env.AWS_LAMBDA_KEY;
        const secretAccessKey = process.env.AWS_LAMBDA_SECRET;
    
        if (!accessKeyId || !secretAccessKey) {
            throw new Error('AWS credentials are missing');
          }
      
        const lambdaClient = new LambdaClient({
          region, 
          credentials: {
            accessKeyId,
            secretAccessKey,
          }
        });
    
        const invokeParams = {
          FunctionName: 'test', 
          InvocationType: 'RequestResponse', 
          Payload: JSON.stringify({ command: ["north","north"] }),
        };
    
        const invokeCommand = new InvokeCommand(invokeParams);
        const response = await lambdaClient.send(invokeCommand);
    
        if (response.FunctionError) {
          console.error('Lambda function error:', response.FunctionError);
          return NextResponse.json({ error: 'Lambda function error' });
        } else {
          const payload = JSON.parse(new TextDecoder().decode(response.Payload));
          return NextResponse.json(payload.msg);
        }
      } catch (error) {
        console.error('Error invoking Lambda function:', error);
        return NextResponse.json({ error: 'Internal server error' });
      }
}