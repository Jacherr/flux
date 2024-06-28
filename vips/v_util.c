#define RETURN_NONZERO(CALL) \
	if (CALL)                \
	{                        \
		return -1;           \
	}