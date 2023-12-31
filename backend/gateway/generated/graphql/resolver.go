package graphql_generated

import "github.com/Archisman-Mridha/instagram-clone/backend/gateway/connectors"

// This file will not be regenerated automatically.
//
// It serves as dependency injection for your app, add any dependencies you require here.

type Resolver struct {
	UsersMicroservice *connectors.UsersMicroserviceConnector
	ProfilesMicroservice *connectors.ProfilesMicroserviceConnector
	FollowshipsMicroservice *connectors.FollowshipsMicroserviceConnector
	PostsMicroservice *connectors.PostsMicroserviceConnector
}