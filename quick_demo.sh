#!/bin/bash

# Vault RPG 快速演示脚本
# 用于录制演示视频时的自动化操作

echo "🎬 Vault RPG 演示开始..."

# 设置环境变量
export VAULT_TOTP_SECRET="JBSWY3DPEHPK3PXPJBSWY3DPEHPK3PXP"
export VAULT_TOTP_ACCOUNT="demo@vaultrpg.com"
export VAULT_TOTP_ISSUER="VaultRPG"

echo "📋 演示数据："
echo "保险库名称: 演示钱包"
echo "主密码: demo2024!@#"
echo "TOTP验证码: $(./target/release/vault_rpg totp code --account "demo@vaultrpg.com" --secret "JBSWY3DPEHPK3PXPJBSWY3DPEHPK3PXP" | grep -o '[0-9]\{6\}')"
echo ""

echo "🎯 开始交互式演示..."
echo "请按照以下步骤操作："
echo "1. 运行: ./target/release/vault_rpg menu"
echo "2. 选择选项2解锁保险库"
echo "3. 输入保险库名称: 演示钱包"
echo "4. 输入主密码: demo2024!@#"
echo "5. 选择谜题类型1 (TOTP验证码)"
echo "6. 输入TOTP验证码: $(./target/release/vault_rpg totp code --account "demo@vaultrpg.com" --secret "JBSWY3DPEHPK3PXPJBSWY3DPEHPK3PXP" | grep -o '[0-9]\{6\}')"
echo ""

# 启动交互式菜单
./target/release/vault_rpg menu
