#!/usr/bin/env bash

set -euxo pipefail

# Patch missing interrupt
for svd in nrf54l15-app.svd  nrf54l10-app.svd nrf54l05-app.svd; do 
	python3 scripts/add_element.py svd/$svd \
	    --xpath ".//peripheral[name='GLOBAL_CLOCK_NS' or name='GLOBAL_POWER_NS' or name='GLOBAL_CLOCK_S' or name='GLOBAL_POWER_S']" \
	    --snippet-file svd-patches/clock_power_irq_nrf54l.xml 
done

for svd in nrf54lm20a-app.svd; do
	python3 scripts/add_element.py svd/$svd \
	    --xpath ".//peripheral[name='GLOBAL_CLOCK_NS' or name='GLOBAL_POWER_NS' or name='GLOBAL_CLOCK_S' or name='GLOBAL_POWER_S']" \
	    --snippet-file svd-patches/clock_power_irq_nrf54lm.xml 
done
