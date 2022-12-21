#include "config.h"
#include <string.h>
#include "expr.h"
#include "parser.h"
#include "error.h"

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
    FILE *f = fopen(in, "re");
    if (!f) {
      err(ERR_FILE_NOT_FOUND, "File '%s' not found!\n", in);
      return;
    }
    expr_apply_from(cfg->expr, f);
  } else {
    cfg->expr = in;
  }
}

void config_finish(Config *cfg) {
  if (cfg->err) {
    return;
  }

  if (!cfg->did_use_file) {
    expr_apply_from(cfg->expr, stdin);
  }
}
