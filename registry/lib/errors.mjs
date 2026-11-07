export class HttpError extends Error {
  constructor(status, code, message, details = undefined) {
    super(message);
    this.status = status;
    this.code = code;
    this.details = details;
  }
}

export const badRequest = (message) => new HttpError(400, 'invalid_request', message);
export const unauthorized = (message = 'authentication required') =>
  new HttpError(401, 'unauthorized', message);
export const forbidden = (message = 'not allowed') => new HttpError(403, 'forbidden', message);
export const notFound = (message = 'not found') => new HttpError(404, 'not_found', message);
export const conflict = (message) => new HttpError(409, 'conflict', message);
export const gone = (message) => new HttpError(410, 'gone', message);
export const tooLarge = (message) => new HttpError(413, 'payload_too_large', message);
