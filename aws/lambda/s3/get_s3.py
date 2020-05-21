import os
import json
import boto3
from boto3.dynamodb.conditions import Key, Attr

dynamodb = boto3.resource('dynamodb')
s3 = boto3.resource('s3')

def lambda_handler(event, context):

    # パスパラメータからユーザー名を取得する
    username = event['pathParameters']['username']

    # DynamoDB (LSI) から username に該当するデータを取得する
    table = dynamodb.Table('photos')
    response = table.query(
        IndexName = 'username-updatetime-index',
        KeyConditionExpression = Key('username').eq(username),
    )

    # 署名付き URL をレスポンスに追加する
    for i, item in enumerate(response['Items']):
        item['preSignedURL'] = s3.meta.client.generate_presigned_url(
            'get_object',
            Params={
                'Bucket': os.environ['BUCKET_NAME'],
                'Key': item['objectkey']
            },
            ExpiresIn=900
        )

    return {
        'statusCode': 200,
        'headers': {
            'Access-Cotrol-Allow-Origin': '*'
        },
        'body': json.dumps(
            { 'Items': response['Items'] }
        ),
    }
n
