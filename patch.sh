#!/usr/bin/env bash

set -euxo pipefail

# Patch missing interrupt
for svd in nrf54l15-app.svd  nrf54l10-app.svd nrf54l05-app.svd; do 
	python3 scripts/add_element.py svd/$svd \
	    --xpath ".//peripheral[name='GLOBAL_CLOCK_NS' or name='GLOBAL_POWER_NS' or name='GLOBAL_CLOCK_S' or name='GLOBAL_POWER_S']" \
	    --snippet-file svd-patches/clock_power_irq_54l.xml 
done

for svd in nrf54lm20a-app.svd; do
	python3 scripts/add_element.py svd/$svd \
	    --xpath ".//peripheral[name='GLOBAL_CLOCK_NS' or name='GLOBAL_POWER_NS' or name='GLOBAL_CLOCK_S' or name='GLOBAL_POWER_S']" \
	    --snippet-file svd-patches/clock_power_irq_54lm.xml
done

# Patch VPR TASKS_TRIGGER and EVENTS_TRIGGERED array sizes from 7 to 23.
# The SVD declares dim=0x7, but the nRF54L VPR hardware has 23 entries
# (indices 0-22). See nrf54l15_types.h: NRF_VPR_Type.TASKS_TRIGGER[23].
# The soft peripheral register interface uses indices 16-20.
for svd in nrf54l15-app.svd nrf54l10-app.svd nrf54l05-app.svd nrf54lm20a-app.svd; do
	python3 scripts/modify_text.py svd/$svd \
	    --xpath ".//peripheral[groupName='VPR']/registers/register[name='TASKS_TRIGGER[%s]']/dim" \
	    --value "0x17"
	python3 scripts/modify_text.py svd/$svd \
	    --xpath ".//peripheral[groupName='VPR']/registers/register[name='EVENTS_TRIGGERED[%s]']/dim" \
	    --value "0x17"
done
