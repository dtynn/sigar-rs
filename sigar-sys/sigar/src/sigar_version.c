#include "sigar.h"

static sigar_version_t sigar_version = {
    "2018-10-31T16:05:46",
    "",
    "2.0.0",
    "x86_64",
    "static",
    "",
    "SIGAR-2.0.0, "
    ""
    "",
    2,
    0,
    0,
    0
};

SIGAR_DECLARE(sigar_version_t *) sigar_version_get(void)
{
    return &sigar_version;
}
