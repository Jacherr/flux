#include <vips/vips.h>
#include "flux_v_util.c"

int v_vips_init()
{
	int i = VIPS_INIT("vips");
	vips_leak_set(TRUE);
	VipsImage *image;
	// cache twemoji font
	vips_text(&image, ".", "fontfile", "/usr/share/fonts/TwemojiCOLR0.otf", NULL);
	g_object_unref(image);
	return i;
}

int v_transcode_to(char *input, size_t len, char **output, size_t *size, char *format)
{
	VipsImage *image = vips_image_new_from_buffer(input, len, "", NULL);

	if (image == NULL)
	{
		return -1;
	}

	RETURN_NONZERO(
		vips_image_write_to_buffer(image, format, (void**) output, size, NULL))

	return 0;
}

char *v_get_error()
{
	return vips_error_buffer();
}

void v_g_free(gpointer ptr)
{
	g_free(ptr);
}