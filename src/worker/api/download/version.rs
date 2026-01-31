use worker::*;



pub async fn get(req: Request, ctx: RouteContext<()>) -> Result<Response> {
    Response::empty()
}