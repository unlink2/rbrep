#ifndef CONFIG_H_
#define CONFIG_H_ 

#include "error.h"
#include "types.h"

typedef struct Config {
  Error err;
} Config;

extern Config cfg;

Config config_init();

#endif 
