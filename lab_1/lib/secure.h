#pragma once

#include <stdio.h>
#include <stdbool.h>

__uint128_t get_hw_id(void);
bool auth_by_hw_id(void);
