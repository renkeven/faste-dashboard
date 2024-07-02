from mangum import Mangum
from litestar import Litestar, get
from litestar.middleware.rate_limit import RateLimitConfig
from litestar.logging import LoggingConfig
import uvicorn

rate_limit_config = RateLimitConfig(rate_limit=("minute", 30), exclude=["/schema"])

logging_config = LoggingConfig(
    root={"level": "INFO", "handlers": ["console"]},
    formatters={
        "standard": {"format": "%(asctime)s - %(name)s - %(levelname)s - %(message)s"}
    },
)

@get("/")
async def hello_world() -> dict[str, str]:
    return {"Welcome to": "Litestartest"}

app = Litestar(
    route_handlers=[hello_world],
    logging_config=logging_config,
    middleware=[
        rate_limit_config.middleware,
    ]
)

# Used to deploy onto cloud platforms like AWS Lambda
handler = Mangum(app, lifespan="off")

if __name__ == "__main__":
    uvicorn.run(app, host="0.0.0.0", port=8080)
