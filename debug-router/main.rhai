fn supergraph_service(service) {
  let response_callback = |response| {
    try {
       response.headers["x-otel-trace-id"] = `${traceid()}`;
    } catch(err) {
       log_error(`span id error: ${err}`);
    }
  };
  service.map_response(response_callback);
}
