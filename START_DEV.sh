#!/bin/bash
# Quick start script for Folivm dev environment

set -e

echo "🚀 Starting Folivm Dev Environment"
echo ""

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Step 1: Build WASM
echo -e "${BLUE}Step 1: Building WASM module...${NC}"
wasm-pack build crates/folivm-wasm --target web --dev
echo -e "${GREEN}✓ WASM built successfully${NC}"
echo ""

# Step 2: Start Vite dev server
echo -e "${BLUE}Step 2: Starting Vite dev server...${NC}"
cd shell
echo -e "${GREEN}✓ Dev server starting at http://localhost:1420${NC}"
echo ""
echo "Press Ctrl+C to stop"
echo ""

npm run dev
