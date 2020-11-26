#import "user_defaults.h"

#import <Foundation/Foundation.h>

#import <stdlib.h>
#import <string.h>

// In their infinite wisdom, these dumb asses decided to return 0 from
// integerForKey and doubleForKey if the key isn't present. Ever hear of
// out-of-band data? So now we have to call objectForKey beforehand to check if
// it's present.
//
// TODO It's not even documented what happens if that value can't be coerced to
// a double or integer. Prolly returns 0 for that too…

bool user_defaults_key_present(NSString *key) {
	@autoreleasepool {
		NSUserDefaults *defaults = [NSUserDefaults standardUserDefaults];
		return [defaults objectForKey:key] != nil;
	}
}

user_defaults_long user_defaults_get_long(const char *key) {
	@autoreleasepool {
		NSString *nsKey = @(key);
		user_defaults_long retval;
		if (user_defaults_key_present(nsKey)) {
			retval.present = true;
			retval.value = [[NSUserDefaults standardUserDefaults] integerForKey:nsKey];
		} else {
			retval.present = false;
			// value intentionally left undefined
		}
		return retval;
	}
}

void user_defaults_set_long(const char *key, long value) {
	@autoreleasepool {
		[[NSUserDefaults standardUserDefaults] setInteger:value forKey:@(key)];
	}
}

user_defaults_double user_defaults_get_double(const char *key) {
	@autoreleasepool {
		NSString *nsKey = @(key);
		user_defaults_double retval;
		if (user_defaults_key_present(nsKey)) {
			retval.present = true;
			retval.value = [[NSUserDefaults standardUserDefaults] doubleForKey:nsKey];
		} else {
			retval.present = false;
			// value intentionally left undefined
		}
		return retval;
	}
}

void user_defaults_set_double(const char *key, double value) {
	@autoreleasepool {
		[[NSUserDefaults standardUserDefaults] setDouble:value forKey:@(key)];
	}
}

char *user_defaults_get_string(const char *key) {
	@autoreleasepool {
		NSString *value = [[NSUserDefaults standardUserDefaults] stringForKey:@(key)];
		if (value) {
			return strndup([value UTF8String], [value length]);
		} else {
			return NULL;
		}
	}
}

void user_defaults_free_string(char *value) {
	free(value);
}

void user_defaults_set_string(const char *key, const char *value) {
	@autoreleasepool {
		[[NSUserDefaults standardUserDefaults] setObject:@(value) forKey:@(key)];
	}
}

user_defaults_string_array *user_defaults_get_string_array(const char *key) {
	@autoreleasepool {
		NSArray<NSString *> *array = [[NSUserDefaults standardUserDefaults] stringArrayForKey:@(key)];
		if (!array)
			return NULL;
		size_t count = [array count]; // TODO Narrowing
		char const **data = malloc(count * sizeof(char const *));
		for (size_t i = 0; i < count; ++i) {
			NSString *value = [array objectAtIndex:i];
			data[i] = strndup([value UTF8String], [value length]);
		}
		user_defaults_string_array *retval = malloc(sizeof(user_defaults_string_array));
		retval->count = count;
		retval->data = data;
		return retval;
	}
}

void user_defaults_free_string_array(user_defaults_string_array *value) {
	for (size_t i = 0; i < value->count; ++i) {
		free((void *)value->data[i]);
	}
	free((void *)value->data);
	free(value);
}

void user_defaults_set_string_array(const char *key, user_defaults_string_array array) {
	@autoreleasepool {
		NSMutableArray *mut_array = [NSMutableArray arrayWithCapacity:array.count];
		for (size_t i = 0; i < array.count; ++i) {
			[mut_array addObject:@(array.data[i])];
		}
		[[NSUserDefaults standardUserDefaults] setObject:mut_array forKey:@(key)];
	}
}
