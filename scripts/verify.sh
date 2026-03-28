#!/bin/bash

echo "Running code format check..."
./scripts/format-check
if [ $? -ne 0 ]; then
    exit 1
fi

echo "Running comment check..."
./scripts/comment-check
if [ $? -ne 0 ]; then
    exit 1
fi

echo "Running linter..."
./scripts/lint
if [ $? -ne 0 ]; then
    exit 1
fi

echo "Running tests..."
./scripts/test
if [ $? -ne 0 ]; then
    exit 1
fi

echo "✅✅✅ All checks passed successfully!"
