#include "config.h"
#include <string.h>

Config cfg;

Config config_init() {
  Config c;
  memset(&c, 0, sizeof(c));

  return c;
}
