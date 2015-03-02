def to_api_version(v):
	assert len(v) >= 2 and v[0] == 'v'
	return v[1:]
