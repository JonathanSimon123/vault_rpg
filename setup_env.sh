#!/bin/bash

# Vault RPG 环境变量设置脚本

echo "🔧 设置 Vault RPG 环境变量..."
echo ""

# 检查是否存在 .env 文件
if [ ! -f ".env" ]; then
    echo "📝 创建 .env 文件..."
    cp env.example .env
    echo "✅ .env 文件已创建"
else
    echo "✅ .env 文件已存在"
fi

echo ""
echo "🔑 设置 TOTP 环境变量..."

# 设置环境变量
export VAULT_TOTP_SECRET="JBSWY3DPEHPK3PXPJBSWY3DPEHPK3PXP"
export VAULT_TOTP_ACCOUNT="demo@vaultrpg.com"
export VAULT_TOTP_ISSUER="VaultRPG"

echo "✅ 环境变量已设置："
echo "   VAULT_TOTP_SECRET: ${VAULT_TOTP_SECRET}"
echo "   VAULT_TOTP_ACCOUNT: ${VAULT_TOTP_ACCOUNT}"
echo "   VAULT_TOTP_ISSUER: ${VAULT_TOTP_ISSUER}"
echo ""

# 测试 TOTP 功能
echo "🧪 测试 TOTP 功能..."
if ./target/release/vault_rpg totp generate --account "demo@vaultrpg.com" --issuer "VaultRPG" --secret "JBSWY3DPEHPK3PXPJBSWY3DPEHPK3PXP"; then
    echo "✅ TOTP 功能测试成功！"
else
    echo "❌ TOTP 功能测试失败"
fi

echo ""
echo "🎯 现在可以运行演示了："
echo "   ./target/release/vault_rpg menu"
echo ""
echo "💡 提示："
echo "   - 选择选项5生成TOTP QR码"
echo "   - 选择选项2解锁保险库"
echo "   - 在解谜时选择TOTP验证码"
