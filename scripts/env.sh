# Helper script that sets all the environment variables needed

echo "Setting environment variables ..."

# Environment
## File-System
export DRAI_CTRL_PATH="~/drai_ctrl"
export DRAI_LOG_PATH="logs"
export DRAI_CONFIG_PATH="config/drake.json"

## Networking
export DRAI_CAMERA_PORT=40324
export DRAI_SERVER_PORT=40325

export DRAI_CAMERA_USER_FILE="/key/drai-camera/username.key"
export DRAI_CAMERA_PW_FILE="/key/drai-camera/password.key"
export DRAI_CTRL_USER_FILE="/key/drai-ctrl/username.key"
export DRAI_CTRL_PW_FILE="/key/drai-ctrl/password.key"
export DRAI_SERVER_USER_FILE="/key/drai-server/username.key"
export DRAI_SERVER_PW_FILE="/key/drai-server/password.key"

# Other
export RUST_LOG=info

# Hardware
## Stepper 
export DRAI_CTRL_VOLTAGE=24

### X-Axis
export DRAI_X_AXIS_STEP_PIN=24
export DRAI_X_AXIS_DIR_PIN=15

export DRAI_X_SWITCH_POS_PIN=23
export DRAI_X_SWITCH_NEG_PIN=0

export DRAI_X_MICROSTEPS=8

### Y-Axis
export DRAI_Y_AXIS_STEP_PIN=5
export DRAI_Y_AXIS_DIR_PIN=25

export DRAI_Y_SWITCH_POS_PIN=12
export DRAI_Y_SWITCH_NEG_PIN=0

export DRAI_Y_MICROSTEPS=8

### Z-Axis
export DRAI_Z_AXIS_STEP_PIN=16
export DRAI_Z_AXIS_DIR_PIN=6

export DRAI_Z_SWITCH_POS_PIN=0
export DRAI_Z_SWITCH_NEG_PIN=19

export DRAI_Z_MICROSTEPS=1

## User-Terminal
export DRAI_UT_SWITCH_START_PIN=26
export DRAI_UT_LED_START_PIN=27

export DRAI_UT_SWITCH_HALT_PIN=21
export DRAI_UT_LED_HALT_PIN=13

echo " -> done!"