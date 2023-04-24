

lumigo-cli powertune-lambda \
    --invocations 5 \
    --region "eu-central-1" \
    --functionName "python-dev-binaryTree" \
    --payload '{"n": 10}' \
    --powerValues 256,512,1024,1792,3008 \
    --strategy "balanced" \
    --parallelInvocation