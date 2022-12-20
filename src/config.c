#include "config.h"
#include <string.h>

Config cfg;

Config config_init() {
  Config c;
  memset(&c, 0, sizeof(c));

  return c;
}

void config_exec(Config *cfg, char *in) {
  if (cfg->err) {
    return;
  }

  if (cfg->expr) {
    // TODO parse file
    cfg->did_use_file = TRUE;
  } else {
    cfg->expr = in;
  }
}

void config_finish(Config *cfg) {
  if (cfg->err) {
    return;
  }

  if (!cfg->did_use_file) {
    // TODO parse stdin
  }
}
