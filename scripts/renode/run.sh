#!/usr/bin/env bash

RESC_STM32L072="targets/stm32l0xx/stm32l072.resc"

RESC=$RESC_STM32L072

# FIXME: Currently 'renode' is a function (or alias) in profile
#        Should think of more permanent solution
source ~/.profile

# Start renode with console (monitor) in the same TTY as this script
# without GUI (USART window) and execute stm32l0x1.resc right away
renode --console --disable-gui -e "i $RESC"
