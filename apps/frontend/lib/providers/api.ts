import type { paths } from 'api-schema';
import createFetchClient from 'openapi-fetch';
import createClient from 'openapi-react-query';

const fetchClient = createFetchClient<paths>({
    baseUrl: '/',
});
export const $api = createClient(fetchClient);
