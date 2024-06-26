# Main setup script, execute after installing

echo
echo "  DrAI - Setup script"
echo =======================
echo " -> ATTENTION: This script must be executed from the projects root folder!"

. "code/scripts/env.sh"

sh "code/scripts/update.sh"

echo Setting correct branches for submodules ...
echo

# syact
cd "code/syact"

git fetch
git checkout enhancements

cd ../..

# sybot
cd "code/sybot"

git fetch
git checkout rework

cd ../..

# Additional directories
mkdir $DRAI_LOG_DIR

echo " => Setup done!"