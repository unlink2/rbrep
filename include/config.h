#ifndef CONFIG_H_
#define CONFIG_H_ 

#include "error.h"
#include "types.h"

typedef struct Config {
  char *expr;

  // if no file was ever used, 
  // use stdin 
  bool did_use_file;
  
  Error err;
} Config;

extern Config cfg;

Config config_init();

void config_exec(Config *cfg, char *in);
void config_finish(Config *cfg);

#endif 
