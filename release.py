import git
import requests
import os

def get_github_releases(repo_owner='NikolaMilosa', repo_name='ntfs-undelete', github_token=None):
    # GitHub API endpoint for releases
    url = f'https://api.github.com/repos/{repo_owner}/{repo_name}/releases'

    headers = {}
    if github_token:
        headers['Authorization'] = f'Bearer {github_token}'

    # Make a request to the GitHub API
    response = requests.get(url, headers=headers)

    # Check if the request was successful (status code 200)
    if response.status_code == 200:
        releases = response.json()
        return releases
    else:
        # Print an error message if the request was not successful
        return None

def get_commits_since_tag(repo_path='.', tag_name=None):
    repo = git.Repo(repo_path)

    if tag_name:
        # If tag_name is provided, get the commit associated with the tag
        try:
            commit = repo.commit(tag_name)
        except git.exc.GitCommandError:
            return None
    else:
        # If tag_name is None, get the latest commit
        commit = repo.head.commit

    # Get all commits from the specified commit until the latest commit
    commits = list(repo.iter_commits(commit, max_count=repo.git.rev_list('HEAD', count=True), reverse=True))

    return commits

commit_prefix = 'https://github.com/NikolaMilosa/ntfs-undelete/commit/'
author_prefix = 'https://github.com/'

github_token = os.environ['GITHUB_TOKEN']
current_tag = os.environ['TAG']

releases = get_github_releases()
tag_name = None
if len(releases) != 0:
    tag_name = sorted(releases, lambda x: x['created_at'])[-1]['tag_name']

release_text = f"""
# Release {current_tag}

### Changelog:
"""

commits = get_commits_since_tag(tag_name=tag_name)
filtered_commits = [
    commit
    for commit in commits
    if commit.parents and  # Check if it's not the first commit (has parents)
    (any(
        file.a_path.startswith('src') or file.b_path.startswith('src') or file.a_path.startswith('Cargo') or file.b_path.startswith('Cargo')
        for file in commit.diff(commit.parents[0]).iter_change_type('M')
    ) or
    any(
        file.a_path.startswith('src') or file.b_path.startswith('src') or file.a_path.startswith('Cargo') or file.b_path.startswith('Cargo')
        for file in commit.diff(commit.parents[0]).iter_change_type('A')
    ) or 
    any(
        file.a_path.startswith('src') or file.b_path.startswith('src') or file.a_path.startswith('Cargo') or file.b_path.startswith('Cargo')
        for file in commit.diff(commit.parents[0]).iter_change_type('D')
    ))
]
for commit in reversed(filtered_commits):
    username = commit.author.email.split('+')[1].split('@')[0]
    release_text += f"* [{commit.hexsha[:7]}]({commit_prefix}{commit.hexsha}) [Author: [{username}]({author_prefix}{username})] - {commit.message.splitlines()[0]}\n"

with open('body', 'w') as f:
    f.write(release_text)
