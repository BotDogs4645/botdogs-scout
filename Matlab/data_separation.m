matchesJSON = fileread("output.json");
jsonSerialized = jsondecode(matchesJSON);
data = struct2table(jsonSerialized);


% Step 2: Data Preprocessing
alliance_scores = data(:, end);
team_scores = data(:, 2:end-1);
num_matches = size(data, 1);

% Step 3: Feature Engineering
avg_team_scores = mean(team_scores, 1);
score_diff = max(team_scores, [], 2) - min(team_scores, [], 2);

% Combine features into a data matrix
X = [team_scores, avg_team_scores', score_diff];

% Step 4: PCA Application
[coeff, score, ~, ~, explained] = pca(X);

% Step 5: Principal Component Selection
num_components = 2; % Set the desired number of principal components
X_pca = score(:, 1:num_components);

% Step 6: Training and Prediction
split_ratio = 0.8; % Split ratio for training and testing
split_idx = round(split_ratio * num_matches);

X_train = X_pca(1:split_idx, :);
y_train = alliance_scores(1:split_idx);

X_test = X_pca(split_idx+1:end, :);
y_test = alliance_scores(split_idx+1:end);

% Train a linear regression model
mdl = fitlm(X_train, y_train);

% Predict scores for new alliances
y_pred = predict(mdl, X_test);

% Step 7: Evaluation
mse = mean((y_pred - y_test).^2);
r2 = mdl.Rsquared.Ordinary;

% Display results
disp(['Mean Squared Error: ', num2str(mse)]);
disp(['R-squared: ', num2str(r2)]);
disp('Predicted scores:');
disp(y_pred);