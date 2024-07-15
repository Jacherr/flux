#include <vips/vips.h>
#include "flux_v_util.c"

int v_gravity(char *input, size_t len, char **output, size_t *size, size_t width, size_t height)
{
	VipsImage *image = vips_image_new_from_buffer(input, len, "", NULL);

	if (image == NULL)
	{
		return -1;
	}

	RETURN_NONZERO(
		vips_gravity(image, &image, VIPS_COMPASS_DIRECTION_CENTRE, (int)width, (int)height, "extend", VIPS_EXTEND_WHITE, NULL))

	if (!vips_image_hasalpha(image))
	{
		RETURN_NONZERO(
			vips_addalpha(image, &image, NULL))
	}

	*output = vips_image_write_to_memory(image, size);

	g_object_unref(image);

	return 0;
}