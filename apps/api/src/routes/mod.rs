pub mod mpesa;
// inside router():
.nest("/api/v1/mpesa", mpesa::router())
