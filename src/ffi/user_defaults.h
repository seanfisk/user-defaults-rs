#pragma once

#include <stdbool.h>
#include <stddef.h>

typedef struct {
	bool present;
	long value; // Only valid if present == true
} user_defaults_long;

user_defaults_long user_defaults_get_long(const char *key);

void user_defaults_set_long(const char *key, long value);

typedef struct {
	bool present;
	double value; // Only valid if present == true
} user_defaults_double;

user_defaults_double user_defaults_get_double(const char *key);

void user_defaults_set_double(const char *key, double value);

/**
 * Return a string from preferences. The string should be deallocated with
 * user_defaults_free_string() when finished.
 */
char *user_defaults_get_string(const char *key);

void user_defaults_free_string(char *value);

void user_defaults_set_string(const char *key, const char *value);

typedef struct {
	size_t count;
	char const *const *data;
} user_defaults_string_array;

/**
 * Return a string array from preferences. The string should be deallocated with
 * user_defaults_free_string_array() when finished.
 */
user_defaults_string_array *user_defaults_get_string_array(const char *key);

void user_defaults_free_string_array(user_defaults_string_array *value);

void user_defaults_set_string_array(const char *key, user_defaults_string_array data);
