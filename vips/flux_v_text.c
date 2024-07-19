#include <vips/vips.h>
#include "flux_v_util.c"

int v_generate_caption_header(char **buf, size_t *size, size_t *height, size_t width, char *text)
{
	VipsImage *image;

	// get size params for text gen
	int fontsize = width / 10;
	int textwidth = ((float)width * .92);

	char *font;
	asprintf(&font, "Twemoji Color Emoji,FuturaExtraBlackCondensed %ipx", fontsize);
	RETURN_NONZERO(
		vips_text(&image, text, "font", font, "rgba", 1, "width", textwidth, "align", VIPS_ALIGN_CENTRE, NULL))

	int im_height = vips_image_get_height(image);

	int new_height = im_height + (width / 6);

	// extend image to correct dimensions
	RETURN_NONZERO(
		vips_gravity(image, &image, VIPS_COMPASS_DIRECTION_CENTRE, (int)width, new_height, "extend", VIPS_EXTEND_WHITE, NULL))

	// generate white overlay image
	VipsImage *overlay;
	RETURN_NONZERO(
		vips_black(&overlay, width, new_height, NULL))
	RETURN_NONZERO(
		vips_invert(overlay, &overlay, NULL))

	VipsImage *comp[] = {image, overlay};

	// overlay image
	VipsImage *output;
	int mode[] = {VIPS_BLEND_MODE_DEST_OVER};
	RETURN_NONZERO(
		vips_composite(comp, &output, 2, mode, NULL))

	// assign return params
	*height = (size_t)new_height;
	*buf = vips_image_write_to_memory(output, size);

	free(font);
	g_object_unref(image);
	g_object_unref(overlay);
	g_object_unref(output);

	return 0;
}

int v_generate_meme_text(char **buf, size_t *size, size_t height, size_t width, char *text)
{
	VipsImage *image;
	VipsImage *mask;

	// old width calculation
	int textwidth = ((float)width * .92);
	int sz = width / 9;
	double radius = (double)sz / 18;

	char *font;
	// asprintf(&font, "Twemoji Color Emoji,Impact %ipx", sz);
	asprintf(&font, "Impact,Twemoji Color Emoji");

	RETURN_NONZERO(
		vips_text(&image, text, "font", font, "rgba", 1, "width", textwidth, "height", height, "align", VIPS_ALIGN_CENTRE, "wrap", VIPS_TEXT_WRAP_WORD_CHAR, NULL))

	RETURN_NONZERO(
		vips_gaussmat(&mask, radius / 2, 0.1, "separable", 1, NULL))

	RETURN_NONZERO(
		vips_linear1(mask, &mask, 8, 0, NULL))

	RETURN_NONZERO(
		vips_embed(image, &image, radius, radius * 2, vips_image_get_width(image) + 2 * radius, (vips_image_get_height(image) + 2 * radius) + (radius * 2), NULL))

	VipsImage *convsep;
	RETURN_NONZERO(
		vips_convsep(image, &convsep, mask, NULL))

	double a[4] = {0, 0, 0, 1};
	double b[4] = {0, 0, 0, 0};
	RETURN_NONZERO(
		vips_linear(convsep, &convsep, a, b, 4, NULL))

	RETURN_NONZERO(
		vips_cast(convsep, &convsep, VIPS_FORMAT_UCHAR, NULL))

	VipsImage *output;
	RETURN_NONZERO(
		vips_composite2(convsep, image, &output, VIPS_BLEND_MODE_OVER, NULL))

	RETURN_NONZERO(
		vips_gravity(output, &output, VIPS_COMPASS_DIRECTION_CENTRE, width, height, "extend", VIPS_EXTEND_BACKGROUND, NULL))

	*buf = vips_image_write_to_memory(output, size);

	free(font);
	g_object_unref(image);
	g_object_unref(mask);
	g_object_unref(convsep);
	g_object_unref(output);

	return 0;
}

int v_generate_motivate_text(char **buf, size_t *size, size_t *height, size_t width, char *text, size_t text_size, int pad_height)
{
	VipsImage *image;
	char *font;
	asprintf(&font, "Twemoji Color Emoji,Times %ipx", text_size);

	RETURN_NONZERO(
		vips_text(&image, text, "font", font, "rgba", 1, "width", width, "align", VIPS_ALIGN_CENTRE, NULL))

	int new_height = pad_height ? vips_image_get_height(image) + (width / 10) : vips_image_get_height(image);

	// extend image to correct dimensions
	RETURN_NONZERO(
		vips_gravity(image, &image, VIPS_COMPASS_DIRECTION_CENTRE, (int)width, new_height, "extend", VIPS_EXTEND_BLACK, NULL))

	VipsImage *overlay;
	RETURN_NONZERO(
		vips_black(&overlay, (int)width, new_height, NULL))

	VipsImage *comp[] = {image, overlay};

	// overlay image
	VipsImage *output;
	int mode[] = {VIPS_BLEND_MODE_DEST_OVER};
	RETURN_NONZERO(
		vips_composite(comp, &output, 2, mode, NULL))

	// assign return params
	*height = (size_t)new_height;
	*buf = vips_image_write_to_memory(output, size);

	free(font);
	g_object_unref(image);
	g_object_unref(overlay);
	g_object_unref(output);

	return 0;
}

int v_generate_heart_locket_text(char **buf, size_t *size, size_t height, size_t width, char *text)
{
	VipsImage *image;
	char *font;
	asprintf(&font, "Twemoji Color Emoji,Times");

	RETURN_NONZERO(
		vips_text(&image, text, "font", font, "rgba", 1, "width", width / 2, "height", height / 2, "align", VIPS_ALIGN_CENTRE, NULL))

	// extend image to correct dimensions
	RETURN_NONZERO(
		vips_gravity(image, &image, VIPS_COMPASS_DIRECTION_CENTRE, (int)width, (int)height, "extend", VIPS_EXTEND_WHITE, NULL))

	// generate white overlay image
	VipsImage *overlay;
	RETURN_NONZERO(
		vips_black(&overlay, width, height, NULL))
	RETURN_NONZERO(
		vips_invert(overlay, &overlay, NULL))

	VipsImage *comp[] = {image, overlay};

	// overlay image
	VipsImage *output;
	int mode[] = {VIPS_BLEND_MODE_DEST_OVER};
	RETURN_NONZERO(
		vips_composite(comp, &output, 2, mode, NULL))

	// assign return params
	*buf = vips_image_write_to_memory(output, size);

	free(font);
	g_object_unref(image);
	g_object_unref(overlay);
	g_object_unref(output);

	return 0;
}