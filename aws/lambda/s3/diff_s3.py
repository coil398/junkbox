import json
import urllib
import boto3
from datetime import datetime

s3 = boto3.client('s3')
dynamodb = boto3.resource('dynamodb')
rekognition = boto3.client('rekognition')

def lambda_handler(event, context):

    bucket = event['Records'][0]['s3']['bucket']['name']
    key = event['Records'][0]['s3']['object']['key']
    key = urllib.parse.unquote(key);
    print("Received event. Bucket: [%s], Key: [%s]" % (bucket, key))

    response = s3.head_object(Bucket=bucket, Key=key)
    username = response['Metadata']['username'];
    print("username : %s" % username)

    # Rekognition でラベルの検出をする
    response = rekognition.detect_labels(
        Image = {
            'S3Object': {
                'Bucket': bucket,
                'Name': key
            }
        }
    )

    # Rekognition から取得したラベルの配列を作成する
    labels = []
    for label in response['Labels']:
        labels.append(label['Name'])

    # DynamoDB にラベルを保存する
    table = dynamodb.Table('photos')
    response = table.put_item(
        Item = {
            'username': username,
            'objectkey': key,
            'labels': labels,
            'updatetime': datetime.now().strftime('%Y%m%d%H%M%S')
        }
    )

