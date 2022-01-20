#!/bin/bash

set -e

./deploy-localnet.sh
cd front
npm i
npm run test

