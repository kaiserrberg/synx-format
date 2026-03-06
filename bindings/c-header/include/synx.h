#ifndef SYNX_H
#define SYNX_H

#ifdef __cplusplus
extern "C" {
#endif

/**
 * Parse a SYNX string and return a JSON string.
 * Caller must free the result with synx_free().
 */
char* synx_parse(const char* input);

/**
 * Parse a SYNX string with engine resolution (!active mode) and return JSON.
 * Caller must free the result with synx_free().
 */
char* synx_parse_active(const char* input);

/**
 * Free a string returned by synx_parse or synx_parse_active.
 */
void synx_free(char* ptr);

#ifdef __cplusplus
}
#endif

#endif /* SYNX_H */
