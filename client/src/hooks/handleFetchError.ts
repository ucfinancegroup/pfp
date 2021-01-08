/**
 * Extract the error from a fetch error object since the OpenAPI generator library doesn't handle it well.
 * If there is no json, then it could have been a 500 error or another network failure, so show the JS error or a
 * fallback.
 * @param e The error object.
 */
export default async function handleFetchError(e: any): Promise<string> {
    if (typeof e.body === 'undefined') throw new Error("Body must be present on error object.");
    const response = new Response(e.body);
    const json = await response.json();

    return json?.message || e.message || "An error occurred";
}
