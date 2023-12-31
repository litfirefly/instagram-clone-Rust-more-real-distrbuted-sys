use super::ports::UsersRepository;
use crate::{
  domain::ports::CreateArgs,
  proto::{AuthenticationResponse, SigninRequest, SignupRequest},
  utils::{hashPassword, jwt::{createJwt, self}, verifyPassword}
};
use anyhow::{anyhow, Result};
use derive_more::Constructor;

const WRONG_PASSWORD_ERROR: &str= "Wrong password provided";

#[derive(Constructor)]
pub struct Usecases {
  usersRepository: &'static dyn UsersRepository
}

impl Usecases {
	pub async fn signup(&self, args: &SignupRequest) -> Result<AuthenticationResponse> {
    let id=
			self.usersRepository
					.create(CreateArgs {
						name: &args.name,
						email: &args.email,
						username: &args.username,
						hashedPassword: &hashPassword(&args.password)?,
					})
					.await?;

    let jwt= createJwt(id)?;
    Ok(AuthenticationResponse { jwt })
  }

  pub async fn signin(&self, args: &SigninRequest) -> Result<AuthenticationResponse> {
    let userDetails= {

			// CASE: User provided his/her email.
			if args.identifier.contains('@') {
        self.usersRepository.findByEmail(&args.identifier).await?}

			// CASE: User provided his/her username.
			else {
        self.usersRepository
          .findByUsername(&args.identifier)
          .await?
      }
    };

    if !verifyPassword(&args.password, &userDetails.hashedPassword)? {
      return Err(anyhow!(WRONG_PASSWORD_ERROR))}

    let jwt= createJwt(userDetails.id)?;
    Ok(AuthenticationResponse { jwt })
  }

	pub async fn verifyJwt(&self, jwt: &str) -> Result<i32> {
		let userId: i32= jwt::decodeJwt(jwt)?
													.parse( ).map_err(|_| anyhow!("Jwt is invalid"))?;

		// TODO: When creating a JWT, include the user's hashed password in it. So here, during JWT
		// verification, we can also verify the user's hashed password.
		let _= self.usersRepository.findById(userId).await?;

		Ok(userId)
	}
}