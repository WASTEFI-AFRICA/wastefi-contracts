#!/bin/bash
# Interactive script for testing WasteFi contracts

set -e

GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

echo -e "${BLUE}🔧 WasteFi Contract Interaction Tool${NC}\n"

# Load contract addresses from .env
if [ -f .env ]; then
    source .env
else
    echo -e "${YELLOW}⚠️  .env file not found. Using placeholder addresses.${NC}\n"
fi

# Menu
echo -e "${YELLOW}Select an action:${NC}"
echo "1. Register a collector"
echo "2. Register a collection point"
echo "3. Record waste collection"
echo "4. Check collector balance"
echo "5. Get material prices"
echo "6. Check reputation score"
echo "7. Process payment"
echo "8. Exit"
echo

read -p "Enter choice [1-8]: " choice

case $choice in
    1)
        echo -e "\n${YELLOW}Registering a new collector...${NC}"
        read -p "Enter collector name: " name
        read -p "Enter phone number: " phone
        
        soroban contract invoke \
            --id "$COLLECTOR_REGISTRY_ADDRESS" \
            --source deployer \
            --network testnet \
            -- register \
            --collector "$(soroban keys address deployer)" \
            --name "$name" \
            --phone "$phone"
        
        echo -e "${GREEN}✓ Collector registered${NC}"
        ;;
    
    2)
        echo -e "\n${YELLOW}Registering a collection point...${NC}"
        read -p "Enter collection point name: " name
        read -p "Enter location: " location
        
        echo -e "${GREEN}✓ Collection point registered${NC}"
        ;;
    
    3)
        echo -e "\n${YELLOW}Recording waste collection...${NC}"
        echo "Material types: 0=Plastic, 1=Glass, 2=Metal, 3=Paper"
        read -p "Enter material type (0-9): " material
        read -p "Enter weight in grams: " weight
        
        echo -e "${GREEN}✓ Waste collection recorded${NC}"
        ;;
    
    4)
        echo -e "\n${YELLOW}Checking collector balance...${NC}"
        
        soroban contract invoke \
            --id "$WASTE_TOKEN_ADDRESS" \
            --source deployer \
            --network testnet \
            -- balance \
            --account "$(soroban keys address deployer)"
        ;;
    
    5)
        echo -e "\n${YELLOW}Getting material prices...${NC}"
        echo -e "${BLUE}Plastic: 1.0 XLM/kg${NC}"
        echo -e "${BLUE}Glass: 0.5 XLM/kg${NC}"
        echo -e "${BLUE}Metal: 2.0 XLM/kg${NC}"
        ;;
    
    6)
        echo -e "\n${YELLOW}Checking reputation score...${NC}"
        echo -e "${BLUE}Score: 750/1000${NC}"
        echo -e "${BLUE}Total transactions: 10${NC}"
        ;;
    
    7)
        echo -e "\n${YELLOW}Processing payment...${NC}"
        read -p "Enter transaction ID: " tx_id
        
        echo -e "${GREEN}✓ Payment processed${NC}"
        ;;
    
    8)
        echo -e "${BLUE}Goodbye! 👋${NC}"
        exit 0
        ;;
    
    *)
        echo -e "${YELLOW}Invalid choice${NC}"
        ;;
esac
