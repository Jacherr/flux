#include <vips/vips.h>
#include "flux_v_util.c"

int v_canny(char *input, size_t len, int width, int height, char **output, size_t *size, double sigma)
{
    VipsImage *image = vips_image_new_from_memory(input, len, width, height, 4, VIPS_FORMAT_UCHAR);

    if (image == NULL)
    {
        return -1;
    }

    RETURN_NONZERO(
        vips_cast(image, &image, VIPS_FORMAT_UCHAR, NULL));

    RETURN_NONZERO(
        vips_canny(image, &image, "sigma", sigma, NULL));

    vips_pngsave(image, "/home/james/stuff/c.png", NULL);

    *output = vips_image_write_to_memory(image, size);

    g_object_unref(image);

    return 0;
}

int v_sobel(char *input, size_t len, int width, int height, char **output, size_t *size)
{
    VipsImage *image = vips_image_new_from_memory(input, len, width, height, 4, VIPS_FORMAT_UCHAR);

    if (image == NULL)
    {
        return -1;
    }

    RETURN_NONZERO(
        vips_sobel(image, &image, NULL));

    RETURN_NONZERO(
        vips_cast(image, &image, VIPS_FORMAT_UCHAR, NULL));

    *output = vips_image_write_to_memory(image, size);

    g_object_unref(image);

    return 0;
}